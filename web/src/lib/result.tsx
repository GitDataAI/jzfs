export interface Result<T> {
    data: T | null;
    code: number;
    msg: string
}