def hello(x):
    print("Hello, {name}"
        .format(name=x))

if __name__ == "__main__":
    names = [
        "Alice",
        "Bob",
        "Charlie"]
    for n in names:
        hello(n)
