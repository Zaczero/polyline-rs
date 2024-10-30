import pytest
from polyline_rs import decode_latlon, decode_lonlat, encode_latlon, encode_lonlat


@pytest.mark.parametrize(
    ('coords', 'precision', 'expected'),
    [
        (
            [(38.5, -120.2), (40.7, -120.95), (43.252, -126.453)],
            5,
            '_p~iF~ps|U_ulLnnqC_mqNvxq`@',
        ),
        (
            [
                (51.77263, 18.08583),
                (51.77261, 18.08586),
                (51.77261, 18.08589),
                (51.77263, 18.08596),
                (51.77269, 18.08604),
                (51.77301, 18.08661),
                (51.77302, 18.08677),
                (51.77302, 18.08702),
            ],
            5,
            '}y~zHmkkmBBE?ECMKO_AqBA_@?q@',
        ),
        (
            [
                (51.772627, 18.085833),
                (51.772611, 18.085859),
                (51.772606, 18.085893),
                (51.772613, 18.085927),
                (51.772628, 18.085957),
                (51.772685, 18.086038),
                (51.773014, 18.086607),
                (51.773023, 18.086775),
                (51.773022, 18.08694),
                (51.77302, 18.08702),
            ],
            6,
            'el}vaBq{zna@^s@HcAMcA]{@qBaDqSqb@QoI@iIB_D',
        ),
    ],
)
def test_encode_decode(coords, precision, expected):
    coords_lonlat = tuple((x, y) for y, x in coords)
    assert encode_latlon(coords, precision) == expected
    assert encode_lonlat(coords_lonlat, precision) == expected
    assert all(
        decoded == pytest.approx(coord, abs=0.1**precision)
        for decoded, coord in zip(decode_latlon(expected, precision), coords)
    )
    assert all(
        decoded == pytest.approx(coord, abs=0.1**precision)
        for decoded, coord in zip(decode_lonlat(expected, precision), coords_lonlat)
    )
