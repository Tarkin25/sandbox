export default interface Todo {
    id: number;
    title: string;
    description: string | null;
    completed: boolean;
}