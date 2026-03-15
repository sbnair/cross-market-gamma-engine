from typing import List, Tuple

def is_monotone(xs: List[float], reverse: bool = False) -> bool:
    if reverse:
        return all(xs[i] >= xs[i + 1] for i in range(len(xs) - 1))
    return all(xs[i] <= xs[i + 1] for i in range(len(xs) - 1))

def check_surface_slice(strikes: List[float], call_prices: List[float]) -> Tuple[bool, str]:
    if len(strikes) != len(call_prices):
        return False, "length mismatch"
    if not is_monotone(strikes):
        return False, "strikes not sorted"
    if any(p < 0 for p in call_prices):
        return False, "negative option price"
    if not is_monotone(call_prices, reverse=True):
        return False, "call prices should generally decrease with strike"
    return True, "ok"

if __name__ == "__main__":
    strikes = [19000, 19500, 20000, 20500, 21000]
    prices = [1700, 1320, 990, 720, 510]
    print(check_surface_slice(strikes, prices))
