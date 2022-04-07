import strmatch


def test_ratio_equal() -> None:
    str_one = 'test string'
    str_two = 'test string'

    assert strmatch.ratio(str_one, str_two) == 1


def test_ratio_case_sensitive() -> None:
    str_one = 'test string'
    str_two = 'test STRING'

    assert strmatch.ratio(str_one, str_two) != 1


def test_distance() -> None:
    str_one = 'test'
    str_two = 'tes'

    assert strmatch.distance(str_one, str_two) == 1


def test_distance_equal() -> None:
    str_one = 'test string'
    str_two = 'test string'

    assert strmatch.distance(str_one, str_two) == 0