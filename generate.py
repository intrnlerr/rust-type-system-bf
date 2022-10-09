
input = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++."

def run(s):
    if not s:
        return "Nil"
    elif s[0] == "+":
        return "Cons<Increment," + run(s[1:]) + ">"
    elif s[0] == "-":
        return "Cons<Decrement," + run(s[1:]) + ">"
    elif s[0] == ">":
        return "Cons<MoveRight," + run(s[1:]) + ">"
    elif s[0] == "<":
        return "Cons<MoveLeft," + run(s[1:]) + ">"
    elif s[0] == ".":
        return "Cons<Output," + run(s[1:]) + ">"
    elif s[0] == "[":
        return "Cons<LoopStart," + run(s[1:]) + ">"
    elif s[0] == "]":
        return "Cons<LoopEnd," + run(s[1:]) + ">"

print(run(input))