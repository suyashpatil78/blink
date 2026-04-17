import { Component, HostListener, ViewChild, ElementRef, AfterViewInit, OnDestroy, OnInit, signal, effect } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";

export interface AppSuggestion {
  name: string;
  desktopPath: string;
}

interface LauncherSearchResult {
  apps: AppSuggestion[];
  calculator: string | null;
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

  calculatorResult = signal<string | null>(null);

  private unlistenFocus?: () => void;

  private debounceTimer?: ReturnType<typeof setTimeout>;

  constructor() {
    effect(() => {
      const value = this.query().trim();

      clearTimeout(this.debounceTimer);

      if (!value) {
        this.appSuggestions.set([]);
        this.calculatorResult.set(null);
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
      const { apps, calculator } = await invoke<LauncherSearchResult>(
        "launcher_search",
        { query }
      );

      this.appSuggestions.set(apps);
      this.calculatorResult.set(calculator ?? null);
    } catch {
      this.appSuggestions.set([]);
      this.calculatorResult.set(null);
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

  async onSubmit(event: SubmitEvent) {
    event.preventDefault();

    const apps = this.appSuggestions();
    const calc = this.calculatorResult();

    if (apps.length === 0 && calc !== null) {
      try {
        await navigator.clipboard.writeText(calc);
      } catch {}

      try {
        await invoke("hide_launcher");
        this.query.set("");
        this.calculatorResult.set(null);
      } catch {}

      return;
    }

    const first = apps[0];

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
    this.calculatorResult.set(null);
  }
}
