from typing import Optional


class BoltzClient:
    """
    BoltzClient class for interacting with the Boltz API.
    """

    def __init__(self, base_url: str, referral_id: Optional[str]) -> None:
        """
        Initialize the Boltz client.

        :param base_url: Boltz server URL
        :param referral_id: Optional referral ID
        """

    def create_submarine_swap(self, asset_from: str, asset_to: str, invoice: str) -> dict:
        """
        Create a submarine swap.

        :param asset_from: Asset to swap from
        :param asset_to: Asset to swap to
        :param invoice: Lightning invoice to pay
        :param amount: Amount to swap
        :return: CreateSubmarineResponse
        """


def new_keys() -> tuple[bytes, bytes]:
    """
    Generate a new key pair for the client.

    :return: tuple with private and public key
    """
