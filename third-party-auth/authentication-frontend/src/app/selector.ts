import { RootState } from "./store";

export type Selector<S, R> = (state: S) => R;

export type SelectorCreator<T> = <R>(selector: Selector<T, R>) => Selector<RootState, R>

export function createSelectorCreator<T>(rootSelector: Selector<RootState, T>): SelectorCreator<T> {
    return <R>(selector: Selector<T, R>) => state => selector(rootSelector(state))
}