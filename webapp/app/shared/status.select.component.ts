import { Component } from "angular2/core";
import { NgIf, NgClass } from "angular2/common";

@Component({
    selector: "status-select",
    template: `
        <div class="status-icon">
          <i class="fa fa-2x" [ngClass]="{'fa-wifi': onAir, 'fa-moon-o': !onAir}"></i>
        </div>
        <div class="status-text">
          <div class="status-operator" *ngIf="activeProfile && activeProfile.operator">
            {{activeProfile.operator}}
          </div>
          <div class="status-message" (click)="toggle()">
            {{status}}
          </div>
        </div>
    `,
    directives: [NgIf, NgClass]
})
export class StatusSelectComponent {
    activeProfile: Object = null;
    onAir: boolean = false;
    status: string = "Off Air";

    constructor() {
        this.activeProfile = {
            operator: "TE0ST"
        };
        this.toggle();
    }

    toggle() {
        this.onAir = !this.onAir;
        this.status = this.onAir ? "On Air" : "Off Air";
    }
}