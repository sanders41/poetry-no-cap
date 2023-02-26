import poetry_no_cap


def main() -> int:
    # print(poetry_no_cap.sum_as_string(5, 20))
    poetry_no_cap.add("fastapi -E all")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
