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

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent implements OnInit, OnDestroy {
  @ViewChild("queryInput") queryInput!: ElementRef<HTMLInputElement>;

  readonly result = signal("");

  private focusUnlisten?: () => void;

  async ngOnInit(): Promise<void> {
    if (!isTauri()) {
      return;
    }
    const unlisten = await getCurrentWindow().onFocusChanged(
      ({ payload: focused }) => {
        if (focused) {
          this.queryInput?.nativeElement?.focus();
          this.queryInput?.nativeElement?.select();
        }
      },
    );
    this.focusUnlisten = unlisten;
  }

  ngOnDestroy(): void {
    this.focusUnlisten?.();
  }

  runQuery(event: SubmitEvent, raw: string): void {
    event.preventDefault();
    const q = raw.trim();
    if (!q.length) {
      return;
    }
    if (!isTauri()) {
      this.result.set(`(browser) you typed: ${q}`);
      return;
    }
    invoke<string>("greet", { name: q }).then((text) => {
      this.result.set(text);
    });
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
