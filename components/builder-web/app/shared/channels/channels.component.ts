import { Component, Input } from "@angular/core";

@Component({
  selector: "hab-channels",
  template: require("./channels.component.html")
})
export class ChannelsComponent {

  @Input() channels: string[];

  expanded: boolean = false;

  stylesFor(channel) {
    let styles: any = {};

    if (!this.expanded) {
      let i = this.channels.indexOf(channel);

      styles.left = `${i * 10}px`;
      styles["z-index"] = (-1 * i);
    }
    else {
      styles.left = `0`;
      styles["z-index"] = 1;
    }

    return styles;
  }

  toggle() {
    this.expanded = !this.expanded;
  }
}
