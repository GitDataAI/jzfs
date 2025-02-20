import {Http} from "@/api/Http.tsx";
import {HotTimeParma} from "@/types.ts";

export class ExploreApi  extends Http {
    async HotRepo(parma: HotTimeParma){
       return await this.patch<string>("/explore/repo", parma)
    }
}