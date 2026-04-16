import { Component, HostListener, ViewChild, ElementRef, AfterViewInit, OnDestroy, OnInit, signal, effect } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
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
  @ViewChild("launcherInput") launcherInput?: ElementRef<HTMLInputElement>;

  query = signal("");

  appSuggestions = signal<AppSuggestion[]>([]);

  private unlistenFocus?: () => void;

  private debounceTimer?: ReturnType<typeof setTimeout>;

  constructor() {
    effect(() => {
      const value = this.query().trim();

      clearTimeout(this.debounceTimer);

      if (!value) {
        this.appSuggestions.set([]);
        return;
      }

      this.debounceTimer = setTimeout(() => {
        this.search(value);
      }, 100);
    });
  }

  async ngOnInit(): Promise<void> {
    try {
      this.unlistenFocus = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
        if (focused) {
          this.launcherInput?.nativeElement.focus();
        }
      });
    } catch {}
  }

  private async search(query: string) {
    try {
      const results = await invoke<AppSuggestion[]>(
        "search_apps_command",
        { query }
      );

      this.appSuggestions.set(results);
    } catch {
      this.appSuggestions.set([]);
    }
  }

  updateQuery(value: string) {
    this.query.set(value);
  }

  showSuggestionList() {
    return !!this.query().trim();
  }

  async launchApp(app: AppSuggestion) {
    try {
      await invoke("launch_desktop_file", {
        path: app.desktopPath,
      });

      await invoke("hide_launcher");
      this.query.set("");
    } catch {}
  }

  onSubmit(event: SubmitEvent) {
    event.preventDefault();

    const first = this.appSuggestions()[0];

    if (first) {
      this.launchApp(first);
    }
  }

  ngOnDestroy() {
    this.unlistenFocus?.();
    clearTimeout(this.debounceTimer);
  }

  @HostListener("keydown.escape")
  closeLauncher() {
    invoke("hide_launcher");
    this.query.set("");
  }
}
