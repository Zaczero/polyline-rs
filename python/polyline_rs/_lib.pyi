from collections.abc import Sequence

def encode_lonlat(coordinates: Sequence[Sequence[float]], precision: int = 5) -> str:
    """
    Encode a sequence of (lon, lat) coordinates to a polyline string.
    """

def encode_latlon(coordinates: Sequence[Sequence[float]], precision: int = 5) -> str:
    """
    Encode a sequence of (lat, lon) coordinates to a polyline string.
    """

def decode_lonlat(polyline: str, precision: int = 5) -> list[tuple[float, float]]:
    """
    Decode a polyline string to a sequence of (lon, lat) coordinates.
    """

def decode_latlon(polyline: str, precision: int = 5) -> list[tuple[float, float]]:
    """
    Decode a polyline string to a sequence of (lat, lon) coordinates.
    """
