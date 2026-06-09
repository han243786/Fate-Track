"""Example front‑end for the BaZi model.

This script demonstrates two ways of interfacing with the `BaZiModel`:

1. Command‑line interface (CLI): you can pass four pillars as arguments
   and it will print a human‑readable report in the console.
2. Streamlit web app (optional): if Streamlit is installed, running
   `streamlit run app.py` will launch a simple web interface where users
   can enter their pillars and receive a report.  Streamlit is not
   required for the CLI functionality.

The pillars must be provided as eight Chinese characters grouped into
four pairs.  For example:

```
python app.py --pillars 甲子 丙申 壬午 庚辰
```

This will parse the four pillar pairs and print the resulting report.

"""

from __future__ import annotations

import argparse
from typing import List, Tuple

from bazi_model import BaZiModel


def parse_pillars(pillar_strs: List[str]) -> List[Tuple[str, str]]:
    """Parse a list of pillar strings into (stem, branch) tuples.

    Each pillar string should consist of exactly two Chinese characters:
    the heavenly stem followed by the earthly branch.  This helper
    function splits the string accordingly.
    """
    pillars: List[Tuple[str, str]] = []
    for p in pillar_strs:
        if len(p) != 2:
            raise ValueError(f"Pillar '{p}' must consist of two Chinese characters")
        pillars.append((p[0], p[1]))
    return pillars


def run_cli(pillars: List[str], gender: str | None = None) -> None:
    """Run the model from the command line and print the report.

    Parameters
    ----------
    pillars : List[str]
        List of four two‑character pillar strings.
    gender : str | None
        Gender of the individual.  Pass "男"或"女"以影响情感和大运解读。
    """
    model = BaZiModel()
    pillar_pairs = parse_pillars(pillars)
    report = model.analyze(pillar_pairs, gender=gender)
    print("四柱:")
    for i, (stem, branch) in enumerate(report.pillars):
        label = ["年柱", "月柱", "日柱", "时柱"][i]
        print(f"  {label}: {stem}{branch}")
    print("\n五行计数:")
    for elem, count in report.five_element_count.items():
        print(f"  {elem}: {count}")
    print(f"\n日主强弱: {report.day_master_strength}")
    print("\n十神分布:")
    for role, count in report.ten_god_distribution.items():
        print(f"  {role}: {count}")
    print("\n分析报告:\n")
    print(report.narrative)


def run_streamlit() -> None:
    """Launch a basic Streamlit app for interactive input.

    This function will import Streamlit on demand.  If Streamlit is not
    installed, it will inform the user gracefully.  To run the app, type
    `streamlit run app.py` in a terminal.
    """
    try:
        import streamlit as st  # type: ignore
    except ImportError:
        print("Streamlit is not installed.  Install it with 'pip install streamlit' to use the web app.")
        return

    st.title("简易八字分析器")
    st.write("请输入四柱，每柱为两个汉字（天干+地支），并选择性别（可选）")
    default = ["甲子", "乙丑", "丙寅", "丁卯"]
    cols = st.columns(4)
    inputs: List[str] = []
    for i, label in enumerate(["年柱", "月柱", "日柱", "时柱"]):
        user_input = cols[i].text_input(label, value=default[i], max_chars=2)
        inputs.append(user_input.strip())
    gender_options = ["", "男", "女"]
    gender = st.selectbox("性别（影响情感、大运解读）", gender_options, index=0)
    if st.button("分析"):
        try:
            pillar_pairs = parse_pillars(inputs)
            model = BaZiModel()
            selected_gender = gender if gender else None
            report = model.analyze(pillar_pairs, gender=selected_gender)
            st.subheader("分析报告")
            st.text(report.narrative)
        except Exception as e:
            st.error(str(e))


def main() -> None:
    parser = argparse.ArgumentParser(description="Run BaZi model analysis")
    parser.add_argument(
        "--pillars",
        nargs=4,
        metavar=("YEAR", "MONTH", "DAY", "HOUR"),
        help="Four pillars (e.g., 甲子 丙申 壬午 庚辰)",
    )
    parser.add_argument(
        "--web", action="store_true", help="Launch the Streamlit web interface"
    )
    parser.add_argument(
        "--gender",
        type=str,
        choices=["男", "女", "male", "female", "M", "F", "m", "f", "boy", "girl"],
        help="Gender of the individual (男 or 女)",
    )
    args = parser.parse_args()
    if args.web:
        run_streamlit()
        return
    if not args.pillars:
        parser.print_help()
        return
    # Normalize gender input
    g = args.gender if args.gender else None
    run_cli(args.pillars, gender=g)


if __name__ == "__main__":
    main()
