from threading import Lock
from typing import Callable


class DiningPhilosophers:
    def __init__(self) -> None:
        self._locks = [Lock() for _ in range(5)]

    # call the functions directly to execute, for example, eat()
    def wantsToEat(
        self,
        philosopher: int,
        pickLeftFork: "Callable[[], None]",
        pickRightFork: "Callable[[], None]",
        eat: "Callable[[], None]",
        putLeftFork: "Callable[[], None]",
        putRightFork: "Callable[[], None]",
    ) -> None:
        if philosopher & 1 == 1:
            right = philosopher
            left = (philosopher + 1) % 5
        else:
            left = philosopher
            right = (philosopher + 1) % 5
        with self._locks[left], self._locks[right]:
            pickLeftFork()
            pickRightFork()
            eat()
            putLeftFork()
            putRightFork()
