// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/contracts/fheETH.sol";

contract fheETHTest is Test {
    FHEToken public fheToken;

    // Create 3 addresses for alice, bob and mallory to test
    address alice = makeAddr("alice");
    address bob = makeAddr("bob");
    address mallory = makeAddr("mallory");
    uint256 public immutable FEE;

    constructor() {
        fheToken = new FHEToken(18, 100);
        FEE = 100;

        // Send 100 ether to alice and bob
        deal(alice, 100 ether);
        deal(bob, 100 ether);

        string memory pk_string = "alice_pk";
        string memory fhe_balance_init = "alice_balance_init";

        // Send the next transaction as alice
        vm.prank(alice);

        // buy_fETH with FEE + FEE + FEE
        (bool sent, ) = address(fheToken).call{value: FEE * 3}(
            abi.encodeWithSignature(
                "buy_fETH(string,string)",
                pk_string,
                fhe_balance_init
            )
        );

        // Assert that the transaction was sent successfully
        assertEq(sent, true);

        // Send the next transaction as bob
        vm.prank(bob);
        (sent, ) = address(fheToken).call{value: FEE * 3}(
            abi.encodeWithSignature(
                "buy_fETH(string,string)",
                pk_string,
                fhe_balance_init
            )
        );

        assertEq(sent, true);
    }

    function test_setup() public {
        // Assert that alice and bob have FEE + FEE as we spent FEE as the fee
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
        string memory fhe_new_balance = "alice_balance";

        uint256 aliceBalance = address(alice).balance;

        // Request to withdraw FEE as alice
        vm.prank(alice);
        (bool sent, ) = address(fheToken).call{value: FEE}(
            abi.encodeWithSignature(
                "withdraw_ETH_request(uint256,string,string,string)",
                FEE,
                fhe_pk_new,
                fhe_sk_old,
                fhe_new_balance
            )
        );

        assertEq(sent, true);
        assertEq(address(alice).balance, aliceBalance - FEE);

        // Approve the withdrawal as the owner
        (sent, ) = address(fheToken).call(
            abi.encodeWithSignature(
                "withdraw_ETH_approved(address,uint256,string,string)",
                alice,
                FEE,
                fhe_pk_new,
                fhe_new_balance
            )
        );

        assertEq(sent, true);
        assertEq(address(alice).balance, aliceBalance);
    }
}
