from typing import Optional


class CreateSubmarineResponse:
    """
    Response object for a submarine swap.
    """
    def __init__(
        self,
        accept_zero_conf: bool,
        address: str,
        bip21: str,
        claim_public_key: bytes,
        expected_amount: int,
        id: str,
        swap_tree: dict,
        blinding_key: Optional[str],
    ) -> None:
        """
        Initialize the CreateSubmarineResponse object.

        :param accept_zero_conf: Whether the swap accepts zero conf
        :param address: Address to send the swap to
        :param bip21: BIP21 string for the swap
        :param claim_public_key: Public key for the swap
        :param expected_amount: Expected amount for the swap
        :param id: Swap ID
        :param swap_tree: Swap tree
        :param blinding_key: Blinding key for the swap
        """


class SwapResponse:
    """
    Response object for a get_pairs.
    """
    def __init__(self, btc: dict[str, dict], lbtc: dict[str, dict]) -> None:
        """
        Initialize the SwapResponse object.

        :param btc: BTC swap parameters
        :param lbtc: LBTC swap parameters
        """


class HeightResponse:
    """
    Response object for a get_height.
    """
    def __init__(self, btc: int, lbtc: int) -> None:
        """
        Initialize the HeightResponse object.

        :param btc: BTC height
        :param lbtc: LBTC height
        """


class Client:
    """
    Client class for interacting with the Boltz API.
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

    def get_pairs(self) -> dict:
        """
        Get the available swap pairs.

        :return: SwapResponse
        """


def new_keys() -> tuple[bytes, bytes]:
    """
    Generate a new key pair for the client.

    :return: tuple with private and public key
    """
