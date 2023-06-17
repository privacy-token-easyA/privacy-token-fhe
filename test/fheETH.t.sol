// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/contracts/fheETH.sol";

contract fheETHTest is Test {
    FHEToken public fheToken;
    address public owner;
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address mallory = makeAddr("mallory");
    uint256 public immutable FEE;

    constructor() {
        fheToken = new FHEToken(18, 100);
        owner = msg.sender;
        FEE = 100;

        deal(alice, 100 ether);
        deal(bob, 100 ether);

        string memory pk_string = "alice_pk";

        vm.prank(alice);

        // Call `buy_tokens` function with the `pk_bytes` parameter
        (bool sent, ) = address(fheToken).call{value: FEE * 3}(
            abi.encodeWithSignature("buy_fETH(string)", pk_string)
        );

        // Assert that the transaction was sent successfully
        assertEq(sent, true);

        vm.prank(bob);
        (sent, ) = address(fheToken).call{value: FEE * 3}(
            abi.encodeWithSignature("buy_fETH(string)", pk_string)
        );

        assertEq(sent, true);
    }

    function test_setup() public {
        assertEq(fheToken.balanceOf(bob), FEE * 2);
        assertEq(fheToken.balanceOf(alice), FEE * 2);
    }

    function test_buy_fETH() public {
        bool aliceExists = fheToken.hasUser(alice);
        assertEq(aliceExists, true);

        bool bobExists = fheToken.hasUser(bob);
        assertEq(bobExists, true);
    }

    function test_send_fhe_tx() public {
        string memory fhe_tx_sender = "alice";
        string memory fhe_tx_receiver = "bob";
        string memory fhe_proof = "proof";

        vm.prank(alice);

        (bool sent, ) = address(fheToken).call{value: FEE}(
            abi.encodeWithSignature(
                "send_fhe_tx(string,string,string)",
                fhe_tx_sender,
                fhe_tx_receiver,
                fhe_proof
            )
        );

        assertEq(sent, true);
    }

    function test_withdraw_ETH() public {
        string memory fhe_sk_old = "alice_sk";
        string memory fhe_pk_new = "alice_pk";

        uint256 aliceBalance = address(alice).balance;

        vm.prank(alice);
        (bool sent, ) = address(fheToken).call{value: FEE}(
            abi.encodeWithSignature(
                "withdraw_ETH_request(uint256,string,string)",
                FEE,
                fhe_pk_new,
                fhe_sk_old
            )
        );

        assertEq(sent, true);
        assertEq(address(alice).balance, aliceBalance - FEE);

        (sent, ) = address(fheToken).call(
            abi.encodeWithSignature(
                "withdraw_ETH_approved(address,uint256,string)",
                alice,
                FEE,
                fhe_pk_new
            )
        );

        assertEq(sent, true);
        assertEq(address(alice).balance, aliceBalance);
    }
}
