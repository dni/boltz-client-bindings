from typing import Optional


class CreateReverseResponse(dict):
    id: str
    invoice: str
    swap_tree: dict
    lockup_address: str
    refund_public_key: bytes
    timeout_block_height: int
    onchain_amount: int
    blinding_key: Optional[str]

    """
    Response object for a reverse swap.
    """
    def __init__(
        self,
        id: str,
        invoice: str,
        swap_tree: dict,
        lockup_address: str,
        refund_public_key: bytes,
        timeout_block_height: int,
        onchain_amount: int,
        blinding_key: Optional[str]
    ) -> None:
        """
        Initialize the CreateReverseResponse object.

        :param id: Swap ID
        :param invoice: Lightning invoice
        :param swap_tree: Swap tree
        :param lockup_address: Lockup address
        :param refund_public_key: Public key for refund
        :param timeout_block_height: Timeout block height
        :param onchain_amount: Onchain amount
        :param blinding_key: Blinding key
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """


class CreateSubmarineResponse(dict):
    accept_zero_conf: bool
    address: str
    bip21: str
    claim_public_key: bytes
    expected_amount: int
    id: str
    swap_tree: dict
    timeout_block_height: int
    blinding_key: Optional[str]
    referral_id: Optional[str]

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
        timeout_block_height: int,
        blinding_key: Optional[str],
        referral_id: Optional[str]
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
        :param timeout_block_height: Timeout block height
        :param blinding_key: Blinding key for the swap
        :param referral_id: Referral ID
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class ReverseLimits(dict):
    maximal: int
    minimal: int

    """
    ReverseLimits object.
    """
    def __init__(self, maximal: int, minimal: int) -> None:
        """
        Initialize the Limits object.

        :param maximal: Maximal amount
        :param minimal: Minimal amount
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class ReverseFees(dict):
    percentage: float
    miner_fees: PairMinerFees

    """
    ReverseFees object.
    """
    def __init__(self, percentage: float, miner_fees: PairMinerFees) -> None:
        """
        Initialize the ReverseFees object.

        :param percentage: Percentage fee
        :param miner_fees: Miner fees
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class ReversePair(dict):
    hash: str
    rate: float
    limits: ReverseLimits
    fees: ReverseFees

    """
    ReversePair object.
    """
    def __init__(self, hash: str, rate: float, limits: ReverseLimits, fees: ReverseFees) -> None:
        """
        Initialize the ReversePair object.

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

class GetReversePairsResponse(dict):
    btc: dict[str, ReversePair]

    __slots__ = ["btc"]

    """
    Response object for a get_reverse_pairs.
    """
    def __init__(self, btc: dict[str, ReversePair], lbtc: dict[str, ReversePair]) -> None:
        """
        Initialize the GetReversePairsResponse object.

        :param btc: BTC swap parameters
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """

class PairMinerFees(dict):
    lockup: int
    claim: int

    """
    PairMinerFees object.
    """
    def __init__(self, lockup: int, claim: int) -> None:
        """
        Initialize the PairMinerFees object.

        :param lockup: Lockup fees
        :param claim: Claim fees
        """

    def to_dict(self) -> dict:
        """
        Convert the response to a dictionary.

        :return: dict
        """


class PairLimits(dict):
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

class SubmarineFees(dict):
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

class SubmarinePair(dict):
    hash: str
    rate: float
    limits: PairLimits
    fees: SubmarineFees

    """
    Swap parameters object.
    """
    def __init__(self, hash: str, rate: float, limits: PairLimits, fees: SubmarineFees) -> None:
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

class GetSubmarinePairsResponse(dict):
    btc: dict[str, SubmarinePair]
    lbtc: dict[str, SubmarinePair]

    __slots__ = ["btc", "lbtc"]

    """
    Response object for a get_submarine_pairs.
    """
    def __init__(self, btc: dict[str, SubmarinePair], lbtc: dict[str, SubmarinePair]) -> None:
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

    def create_submarine_swap(self, asset_from: str, asset_to: str, invoice: str, public_key: bytes, pair_hash: Optional[str] = None) -> CreateSubmarineResponse:
        """
        Create a submarine swap.

        :param asset_from: Asset to swap from
        :param asset_to: Asset to swap to
        :param invoice: Lightning invoice to pay
        :param public_key: Public key for the swap
        :param pair_hash: Hash of the swap pair
        :return: CreateSubmarineResponse
        """

    def create_reverse_swap(self, asset_from: str, asset_to: str, amount: int, public_key: bytes, pair_hash: Optional[str] = None) -> CreateReverseResponse:
        """
        Create a reverse swap.

        :param asset_from: Asset to swap from
        :param asset_to: Asset to swap to
        :param amount: Amount to swap
        :param public_key: Public key for the swap
        :param pair_hash: Hash of the swap pair
        :return: CreateReverseResponse
        """

    def get_submarine_pairs(self) -> GetSubmarinePairsResponse:
        """
        Get the available swap pairs.

        :return: GetSubmarinePairsResponse
        """

    def get_reverse_pairs(self) -> GetReversePairsResponse:
        """
        Get the available reverse swap pairs.

        :return: GetReversePairsResponse
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
    :param network: either "main", "testnet" or "regtest"
    :param address: Bitcoin address
    :return: bool
    """


class BtcSwapScript:
    """
    BtcSwapScript object.
    """
    def __init__(self, script: dict) -> None:
        """
        Initialize the BtcSwapScript object.

        :param script: Swap script
        """

    @staticmethod
    def from_submarine_response(created_response: CreateSubmarineResponse, our_pubkey: bytes) -> BtcSwapScript:
        """
        Create a BtcSwapScript object from a submarine response.

        :param created_response: CreateSubmarineResponse
        :param our_pubkey: Our public key
        :return: BtcSwapScript
        """

    def is_submarine(self) -> bool:
        """
        Check if the swap is a submarine swap.

        :return: bool
        """
