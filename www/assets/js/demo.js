const currentDate = new Date().toLocaleString("en-US", {
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
});
const currentTime = new Date().toLocaleString("en-US", {
  hour: "2-digit",
  minute: "2-digit",
  second: "2-digit",
});

class TerminalDemo {
  constructor() {
    this.terminal = document.getElementById("demo-terminal");
    this.commands = [
      {
        prompt: `[${currentTime}] puppy@cage in ~/source/github.com/0x800a6/flux (master)`,
        command: "ls -la",
        output: `total 42\ndrwxr-xr-x  15 puppy  puppy   480 Mar 14 10:23 .`,
      },
      {
        prompt: `[${currentTime}] puppy@cage in ~/source/github.com/0x800a6/flux (master)`,
        command: "git status",
        output:
          "On branch master\nYour branch is up to date with 'origin/master'",
      },
    ];
    this.init();
  }

  formatPrompt(prompt) {
    // Split the prompt into parts for colorization
    const parts = prompt.match(/(\[.*?\])(.*? in )(.*?)( \(.*?\))/);
    if (parts) {
      return `<span class="time-bracket">${parts[1]}</span>${parts[2]}<span class="directory-text">${parts[3]}</span><span class="git-branch">${parts[4]}</span>`;
    }
    return prompt;
  }

  async typePrompt(prompt) {
    const promptElement = document.createElement("div");
    promptElement.className = "terminal-prompt";
    promptElement.innerHTML = this.formatPrompt(prompt) + "\nλ ";
    this.terminal.appendChild(promptElement);
    await this.sleep(500);
  }

  async typeCommand(command) {
    const commandElement = document.createElement("span");
    commandElement.className = "terminal-command";
    this.terminal.appendChild(commandElement);

    for (const char of command) {
      commandElement.textContent += char;
      await this.sleep(Math.random() * 100 + 50);
    }
    this.terminal.appendChild(document.createElement("br"));
    await this.sleep(500);
  }

  async showOutput(output) {
    if (output) {
      const outputElement = document.createElement("div");
      outputElement.className = "terminal-output";
      outputElement.textContent = output;
      this.terminal.appendChild(outputElement);
      await this.sleep(1000);
    }
  }

  async typeCommands() {
    for (const cmd of this.commands) {
      await this.typePrompt(cmd.prompt);
      await this.typeCommand(cmd.command);
      await this.showOutput(cmd.output);
    }
    // Restart demo after completion
    setTimeout(() => {
      this.terminal.innerHTML = "";
      this.typeCommands();
    }, 3000);
  }

  sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  init() {
    this.typeCommands();
  }
}

// Initialize the demo when the DOM is loaded
document.addEventListener("DOMContentLoaded", () => {
  new TerminalDemo();
});
