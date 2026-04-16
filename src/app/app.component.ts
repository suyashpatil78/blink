import { Component, HostListener } from "@angular/core";
import { RouterOutlet } from "@angular/router";
import { invoke } from "@tauri-apps/api/core";

@Component({
  selector: "app-root",
  imports: [RouterOutlet],
  templateUrl: "./app.component.html",
  styleUrl: "./app.component.css",
})
export class AppComponent {
  greetingMessage = "";

  onSubmit(event: SubmitEvent): void {
    event.preventDefault();

    const command = (event.target as HTMLFormElement).querySelector<HTMLInputElement>("#greet-input")?.value;
    if (command) {
      console.log(command);
    }
  }

  @HostListener("keydown.escape")
  onEscape(): void {
    invoke("close_window");
  }
}
