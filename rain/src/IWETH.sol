pragma solidity ^0.8.18;

import { IERC20Metadata } from 'openzeppelin-contracts/contracts/token/ERC20/extensions/IERC20Metadata.sol';

interface IWETH is IERC20Metadata {
    event Deposit(address indexed dst, uint256 wad);
    event Withdrawal(address indexed src, uint256 wad);

    function deposit() external payable;
    function withdraw(uint256 wad) external;
    function withdraw(uint256 wad, address user) external;
}