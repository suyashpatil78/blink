import { Component, HostListener, ViewChild, ElementRef, AfterViewInit, OnDestroy, OnInit } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent implements AfterViewInit, OnInit, OnDestroy {
  @ViewChild("launcherInput") launcherInput?: ElementRef<HTMLInputElement>;

  query = "";

  private unlistenFocus?: () => void;

  async ngOnInit(): Promise<void> {
    try {
      const { getCurrentWindow } = await import("@tauri-apps/api/window");
      this.unlistenFocus = await getCurrentWindow().onFocusChanged(({ payload: focused }) => {
        if (focused) {
          queueMicrotask(() => this.launcherInput?.nativeElement.focus());
        }
      });
    } catch {
      /* not running inside the Tauri webview */
    }
  }

  ngAfterViewInit(): void {
    this.launcherInput?.nativeElement.focus();
  }

  ngOnDestroy(): void {
    this.unlistenFocus?.();
  }

  onSubmit(event: SubmitEvent): void {
    event.preventDefault();
    const trimmed = this.query.trim();
    if (trimmed) {
      console.log(trimmed);
    }
  }

  @HostListener("keydown.escape")
  onEscape(): void {
    void invoke("hide_launcher");
  }
}
