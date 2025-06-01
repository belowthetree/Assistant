import { ModelBase } from "../model/modelbase";
import { Business } from "./business/business";
import { Theme } from "./business/theme";
import { RoleCard } from "./role/rolecard";
import { Talk } from "./talk/talk";

export enum EAssistantMode {
  // 直接输出结果
  Direct,
  // 要求分步完成
  Step,
}

export class Assistant {
  role: RoleCard;
  business: Business;
  talk: Talk;
  mode: EAssistantMode;

  constructor(
    role: RoleCard,
    model: ModelBase,
    mode: EAssistantMode = EAssistantMode.Direct,
  ) {
    this.role = role;
    this.mode = mode;
    this.talk = new Talk(model, role.prompt);
  }

  startBusiness(theme: Theme) {
    this.talk.userSay(theme.getSystemPrompt()).then((_) => {});
  }
}
