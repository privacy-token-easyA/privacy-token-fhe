// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "lib/solmate/src/tokens/ERC20.sol";

contract FHEToken is ERC20 {
    modifier onlyUser() {
        require(hasUser[msg.sender] == true, "FHEToken: sender is not a user");
        _;
    }

    modifier onlyOwner() {
        require(
            msg.sender == owner,
            "FHEToken: only owner can call this function"
        );
        _;
    }

    modifier onlyValidFees() {
        require(msg.value >= FEE, "FHEToken: msg.value isnt geq FEE");
        total_fees += FEE;
        _;
    }

    /**
     * @dev Emitted when a user buys fETH
     * @param from The address of the user who bought fETH
     * @param amount The amount of fETH bought
     * @param fhe_pk The fhe public key of the user
     * @param fhe_balance_init The initial balance of the user in the fhe_account
     */
    event Deposit_fETH(
        address indexed from,
        uint256 amount,
        string fhe_pk,
        string fhe_balance_init
    );

    /**
     * @dev Emitted when a user sends a transaction
     * @param from The address of the user who sent the transaction
     * @param fhe_tx_hash The hash of the transaction that acts as the transaction id
     * @param fhe_tx_sender The sender_fhe_tx  of the transaction (this is like the return transaction and sends the tokens back to the sender)
     * @param fhe_tx_receiver The receiver_fhe_tx of the transaction
     * @param fhe_proof The proof of the transaction
     */
    event Send_fhe_tx(
        address indexed from,
        bytes32 fhe_tx_hash,
        string fhe_tx_sender,
        string fhe_tx_receiver,
        string fhe_proof
    );

    /**
     * @dev Emitted when a user requests for a withdrawal
     * @param to The address of the user who requested the withdrawal
     * @param amount The amount of ETH requested to be withdrawn
     * @param fhe_pk_new The new fhe public key of the user
     * @param fhe_sk_old The old fhe secret key of the user
     */
    event Withdraw_ETH_Request(
        address indexed to,
        uint256 amount,
        string fhe_pk_new,
        string fhe_sk_old,
        string fhe_new_balance
    );

    /**
     * @dev Emitted when a user withdraws ETH
     * @param to The address of the user who withdrew ETH
     * @param amount The amount of ETH withdrawn
     * @param fhe_pk_new The new fhe public key of the user
     */
    event Withdraw_ETH_Approved(
        address indexed to,
        uint256 amount,
        string fhe_pk_new,
        string fhe_new_balance
    );

    address payable public owner;
    uint256 public immutable FEE;
    uint256 public total_fees;
    mapping(address => bool) public hasUser;

    constructor(
        uint8 _decimals,
        uint256 _fee
    ) ERC20("FHEToken", "FHT", _decimals) {
        owner = payable(msg.sender);
        hasUser[msg.sender] = true;

        FEE = _fee;
        total_fees = 0;
    }

    /**
     * @dev Mints fETH to the msg.sender
     * @param _fhe_pk The public key of the user
     */
    function deposit_fETH(
        string calldata _fhe_pk,
        string calldata _fhe_balance_init
    ) public payable onlyValidFees {
        _mint(msg.sender, msg.value - FEE);

        // If the sender is not a user, add them to the list of users
        if (hasUser[msg.sender] == false) {
            hasUser[msg.sender] = true;
        }

        emit Deposit_fETH(
            msg.sender,
            msg.value - FEE,
            _fhe_pk,
            _fhe_balance_init
        );
    }

    /**
     * @dev Sends a transaction to the fhe_account
     * @param _fhe_tx_sender The sender_fhe_tx of the transaction  (generated by the user's node)
     * @param _fhe_tx_receiver The receiver_fhe_tx of the transaction (generated by the user's node)
     * @param _fhe_proof The proof of the transaction (generated by the user's node and verified by the fhe_node)
     */
    function send_fhe_tx(
        string calldata _fhe_tx_sender,
        string calldata _fhe_tx_receiver,
        string calldata _fhe_proof
    ) external payable onlyUser onlyValidFees {
        // generate the hash of the transaction
        bytes32 _fhe_tx_hash = keccak256(
            abi.encodePacked(
                msg.sender,
                _fhe_tx_sender,
                _fhe_tx_receiver,
                block.number
            )
        );

        emit Send_fhe_tx(
            msg.sender,
            _fhe_tx_hash,
            _fhe_tx_sender,
            _fhe_tx_receiver,
            _fhe_proof
        );
    }

    /**
     * @dev Transfers the ETH from the fhe_account to the msg.sender
     * @param _amount The amount of ETH to be withdrawn. This needs to be equal to the tokens owner in the fhe_account with _sk as the secret key
     * @param _fhe_sk The secret key used to encrypt the new public key
     * @param _new_fhe_pk The new public key to be used for future transactions
     * @param _fhe_new_balance The new balance of the user in the fhe_account
     */
    function withdraw_ETH_request(
        uint256 _amount,
        string calldata _fhe_sk,
        string calldata _new_fhe_pk,
        string calldata _fhe_new_balance
    ) external payable onlyUser onlyValidFees {
        emit Withdraw_ETH_Request(
            msg.sender,
            _amount,
            _fhe_sk,
            _new_fhe_pk,
            _fhe_new_balance
        );
    }

    function withdraw_ETH_approved(
        address _user,
        uint256 _amount,
        string calldata _new_fhe_pk,
        string calldata _fhe_new_balance
    ) external payable onlyOwner {
        payable(_user).transfer(_amount);

        emit Deposit_fETH(_user, 0, _new_fhe_pk, _fhe_new_balance);
        emit Withdraw_ETH_Approved(
            _user,
            _amount,
            _new_fhe_pk,
            _fhe_new_balance
        );
    }

    function changeOwner(address payable _owner) external onlyOwner {
        owner = _owner;
    }

    function withdrawFees() external onlyOwner {
        payable(owner).transfer(total_fees);
        total_fees = 0;
    }

    event ReveivedEther(address indexed from, uint256 amount);

    receive() external payable {
        emit ReveivedEther(msg.sender, msg.value);
    }
}
