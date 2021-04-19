type Func<I, O> = (input: I) => O;
type BiFunc<I1, I2, O> = (input1: I1, input2: I2) => O;

type Bool = <T>(a: T, b: T) => T;

const TRUE: Bool = (a, b) => a;
const FALSE: Bool = (a, b) => b;
const BOOL = (b: any) => b ? TRUE : FALSE;

type BoolOperator = (b: Bool) => Bool;
type BoolCombinator = (a: Bool, b: Bool) => Bool;

const NOT: BoolOperator = b => b(FALSE, TRUE);
const AND: BoolCombinator = (a, b) => a(b, FALSE);
const OR: BoolCombinator = (a, b) => a(TRUE, b);

console.log("not true", NOT(TRUE));
console.log("not false", NOT(FALSE));
console.log("true and true", AND(TRUE, TRUE));
console.log("true and false", AND(TRUE, FALSE));
console.log("false and false", AND(FALSE, FALSE));
console.log("false and true", AND(FALSE, TRUE));
console.log("true or true", OR(TRUE, TRUE));
console.log("true or false", OR(TRUE, FALSE));
console.log("false or false", OR(FALSE, FALSE));
console.log("false or true", OR(FALSE, TRUE));

type Predicate<T> = (input: T) => Bool;

const loop = <T>(start: T, condition: Predicate<T>, increment: Func<T, T>) => (action: Func<T, void>) => {
    for(let current = start; condition(current) === TRUE; current = increment(current)) {
        action(current);
    }
}

const loopAsync = <T>(start: T, condition: Predicate<T>, increment: Func<T, T>) => async (action: Func<T, Promise<void>>) => {
    for(let current = start; condition(current) === TRUE; current = increment(current)) {
        await action(current);
    }
}

const lessThan = (value: number) => (n: number) => BOOL(n < value);
const increment = (amount: number) => (n: number) => n + amount;

const repeat = (amount: number) => loop(0, lessThan(amount), increment(1));

repeat(10)(i => console.log(i));

const repeatAsync = (amount: number) => loopAsync(0, lessThan(amount), increment(1))

const asyncAction = async (i: number) => console.log(i);

repeatAsync(10)(asyncAction).then(() => console.log("async repeat finished"));
