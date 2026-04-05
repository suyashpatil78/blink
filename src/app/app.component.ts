import {
  Component,
  ElementRef,
  HostListener,
  OnDestroy,
  OnInit,
  ViewChild,
  signal,
} from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke, isTauri } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export interface AppSuggestion {
  name: string;
  desktopPath: string;
}

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent implements OnInit, OnDestroy {
  @ViewChild("queryInput") queryInput!: ElementRef<HTMLInputElement>;

  readonly suggestions = signal<AppSuggestion[]>([]);
  readonly selectedIndex = signal(0);
  readonly errorHint = signal("");

  private focusUnlisten?: () => void;
  private searchDebounce?: ReturnType<typeof setTimeout>;

  async ngOnInit(): Promise<void> {
    if (!isTauri()) {
      return;
    }
    const unlisten = await getCurrentWindow().onFocusChanged(
      ({ payload: focused }) => {
        if (focused) {
          queueMicrotask(() => {
            this.queryInput?.nativeElement?.focus();
            this.queryInput?.nativeElement?.select();
          });
        }
      },
    );
    this.focusUnlisten = unlisten;
  }

  ngOnDestroy(): void {
    this.focusUnlisten?.();
    clearTimeout(this.searchDebounce);
  }

  onQueryInput(raw: string): void {
    this.errorHint.set("");
    clearTimeout(this.searchDebounce);
    this.searchDebounce = setTimeout(() => void this.refreshSuggestions(raw), 90);
  }

  private async refreshSuggestions(raw: string): Promise<void> {
    const q = raw.trim();
    if (!q.length) {
      this.suggestions.set([]);
      this.selectedIndex.set(0);
      return;
    }
    if (!isTauri()) {
      this.suggestions.set([
        { name: "Example App (browser mock)", desktopPath: "" },
      ]);
      this.selectedIndex.set(0);
      return;
    }
    try {
      const list = await invoke<AppSuggestion[]>("search_apps", { query: q });
      this.suggestions.set(list);
      this.selectedIndex.set(0);
    } catch (e) {
      this.suggestions.set([]);
      this.selectedIndex.set(0);
      this.errorHint.set(
        e instanceof Error ? e.message : "Could not load suggestions.",
      );
    }
  }

  async onSubmit(event: SubmitEvent): Promise<void> {
    event.preventDefault();
    const list = this.suggestions();
    if (!list.length) {
      return;
    }
    const i = Math.min(this.selectedIndex(), list.length - 1);
    await this.launchAt(i);
  }

  private async launchAt(index: number): Promise<void> {
    const list = this.suggestions();
    const app = list[index];
    if (!app?.desktopPath) {
      if (!isTauri()) {
        this.errorHint.set("Run inside Blink (Tauri) to launch apps.");
      }
      return;
    }
    this.errorHint.set("");
    try {
      await invoke("launch_app", { desktopFilePath: app.desktopPath });
      this.queryInput.nativeElement.value = "";
      this.suggestions.set([]);
      this.selectedIndex.set(0);
      await invoke("hide_main_window");
    } catch (e) {
      this.errorHint.set(
        e instanceof Error ? e.message : "Could not start application.",
      );
    }
  }

  onSuggestionPick(app: AppSuggestion, event: MouseEvent): void {
    event.preventDefault();
    const i = this.suggestions().findIndex((a) => a.desktopPath === app.desktopPath);
    if (i >= 0) {
      void this.launchAt(i);
    }
  }

  @HostListener("document:keydown", ["$event"])
  handleArrowNav(event: Event): void {
    const e = event as KeyboardEvent;
    const list = this.suggestions();
    if (!list.length) {
      return;
    }
    if (e.key === "ArrowDown") {
      e.preventDefault();
      this.selectedIndex.update((i) => Math.min(i + 1, list.length - 1));
    } else if (e.key === "ArrowUp") {
      e.preventDefault();
      this.selectedIndex.update((i) => Math.max(i - 1, 0));
    }
  }

  @HostListener("document:keydown.escape", ["$event"])
  async handleEscape(event: Event): Promise<void> {
    event.preventDefault();
    if (!isTauri()) {
      return;
    }
    await invoke("hide_main_window");
  }
}
