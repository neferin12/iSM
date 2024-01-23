#!/usr/bin/env python3

import typer
from typing_extensions import Annotated

from ism_io import import_seminars, import_students, print_students

from Algorithm import run_algorithm


# def main(wahldatei: str, seminardatei: str, iterations: int) -> None:
def main(votes: Annotated[str, typer.Argument(help="path to the file with the students' votes")],
         seminars: Annotated[str, typer.Argument(help="path to the file with the seminars' data")],
         iterations: Annotated[int, typer.Argument(help="number of iterations to run the algorithm")]) -> None:
    """
    This is a program to assign students to seminars.
    """
    seminars_values = import_seminars(seminars)
    vote_values = import_students(votes, seminars_values)

    iteration = run_algorithm(vote_values, iterations)

    print_students(vote_values, iteration)


if __name__ == '__main__':
    typer.run(main)
