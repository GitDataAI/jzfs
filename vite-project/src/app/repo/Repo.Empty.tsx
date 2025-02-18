import {ImFileEmpty} from "react-icons/im";

export const RepoEmpty = () => {
    return(
        <div className="repo-empty">
            <ImFileEmpty />
            <p>No files in this repository</p>
        </div>
    )
}