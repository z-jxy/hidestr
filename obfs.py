import os


def remove_comments(lines: list[str]):
    output = []
    for line in lines:
        if "//" in line:
            idx = line.index("//")

            if any(x in line for x in ["'", '"']):
                q_idx = line.find("'")
                if q_idx != -1:
                    rQ = line.find("'", q_idx + 1)
                    if q_idx < idx < rQ:
                        output.append(line)
                        continue

                q_idx = line.find('"')
                if q_idx != -1:
                    rQ = line.find('"', q_idx + 1)
                    if q_idx < idx < rQ:
                        output.append(line)
                        continue

            output.append(line[:idx].strip() + "\n")
            continue
        output.append(line)
    return "".join([x for x in output if x != "\n"])


def save(filePath, content):
    with open(filePath, "w") as f:
        f.write(content)


def main():
    # walk the directory
    for root, _, files in os.walk("SharpSploit"):
        for file in files:
            if file.endswith(".cs") and not file.endswith(".obfs.cs"):
                filePath = os.path.join(root, file)
                with open(filePath, "r") as f:
                    lines = f.readlines()
                content = remove_comments(lines)
                save(f"{filePath}.obfs.cs", content)
                print(f"[*] Removed comments from {filePath}")


if __name__ == "__main__":
    main()
