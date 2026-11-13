"""Generate `docs/figures/python_bindings.svg` through the Python binding.

This is the Python counterpart of the Rust examples: it uses the very same
recipe builders through `scienceview_python`, so the figure proves the binding
end to end rather than illustrating it by hand.

Run it with the wheel installed:

    python -m pip install target/wheels/scienceview-*.whl
    python examples/python_bindings.py
"""

from pathlib import Path

import scienceview as sv

SEPAL_WIDTH = [1.4, 1.7, 1.9, 1.5, 2.0, 1.6, 2.6, 3.0, 2.8, 3.2, 2.7, 3.4, 3.1, 3.5, 3.3, 3.7, 3.9, 4.4]
PETAL_LENGTH = [2.6, 3.1, 3.4, 2.9, 3.8, 3.0, 4.3, 5.1, 4.7, 5.6, 4.5, 6.0, 5.4, 6.3, 5.8, 6.7, 7.1, 7.6]
SPECIES = [0.0] * 6 + [1.0] * 6 + [2.0] * 6


def main() -> None:
    figure = sv.scatter_plot(
        {"sepal_width": SEPAL_WIDTH, "petal_length": PETAL_LENGTH, "species": SPECIES},
        backend="svg",
        x="sepal_width",
        y="petal_length",
        series="species",
        title="Bound scatter: sepal width against petal length",
        x_label="sepal width",
        y_label="petal length",
    )

    destination = Path(__file__).resolve().parent.parent / "docs" / "figures" / "python_bindings.svg"
    destination.write_bytes(figure.render())
    print(f"wrote {destination} ({destination.stat().st_size} bytes)")


if __name__ == "__main__":
    main()
