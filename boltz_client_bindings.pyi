from typing import Optional


class CreateSubmarineResponse(dict):
    accept_zero_conf: bool
    address: str
    bip21: str
    claim_public_key: bytes
    expected_amount: int
    id: str
    swap_tree: dict
    blinding_key: Optional[str]

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

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class Limits(dict):
    maximal: int
    minimal: int
    maximal_zero_conf: int

    """
    Limits object.
    """
    def __init__(self, maximal: int, minimal: int, maximal_zero_conf: int) -> None:
        """
        Initialize the Limits object.

        :param maximal: Maximal amount
        :param minimal: Minimal amount
        :param maximal_zero_conf: Maximal zero conf amount
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class Fees(dict):
    percentage: float
    miner_fees: int

    """
    Fees object.
    """
    def __init__(self, percentage: float, miner_fees: int) -> None:
        """
        Initialize the Fees object.

        :param percentage: Percentage fee
        :param miner_fees: Miner fees
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class SwapParams(dict):
    hash: str
    rate: float
    limits: Limits
    fees: Fees

    """
    Swap parameters object.
    """
    def __init__(self, hash: str, rate: float, limits: Limits, fees: Fees) -> None:
        """
        Initialize the SwapParams object.

        :param hash: Hash of the swap
        :param rate: Rate of the swap
        :param limits: Limits of the swap
        :param fees: Fees of the swap
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class SwapResponse(dict):
    btc: dict[str, SwapParams]
    lbtc: dict[str, SwapParams]

    __slots__ = ["btc", "lbtc"]

    """
    Response object for a get_pairs.
    """
    def __init__(self, btc: dict[str, SwapParams], lbtc: dict[str, SwapParams]) -> None:
        """
        Initialize the SwapResponse object.

        :param btc: BTC swap parameters
        :param lbtc: LBTC swap parameters
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """


class HeightResponse(dict):
    """
    Response object for a get_height.
    """
    def __init__(self, btc: int, lbtc: int) -> None:
        """
        Initialize the HeightResponse object.

        :param btc: BTC height
        :param lbtc: LBTC height
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
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

    def get_pairs(self) -> SwapResponse:
        """
        Get the available swap pairs.

        :return: SwapResponse
        """

    def get_height(self) -> HeightResponse:
        """
        Get the current block height.

        :return: HeightResponse
        """


def new_keys() -> tuple[bytes, bytes]:
    """
    Generate a new key pair for the client.

    :return: tuple with private and public key
    """


def validate_address(chain: str, network: str, address: str) -> bool:
    """
    Validate an onchain address.

    :param chain: either "BTC" or "L-BTC"
    :param network: either "mainnet", "testnet" or "regtest"
    :param address: Bitcoin address
    :return: bool
    """
