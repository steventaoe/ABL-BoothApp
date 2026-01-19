/**
 * 时间格式化工具测试
 * 测试 UTC 时间到 UTC+8 的转换
 */

import { formatTimestamp, formatDate, formatTime } from './dateFormatter';

// 测试用例
const testCases = [
  {
    input: '2024-01-19 08:30:00',  // UTC 时间
    expected: '2024/01/19 16:30:00',  // UTC+8 应该是 16:30
    description: 'UTC 8:30 应该转换为 UTC+8 16:30'
  },
  {
    input: '2024-01-19 16:00:00Z',  // 明确标记为 UTC
    expected: '2024/01/20 00:00:00',  // UTC+8 应该是次日 00:00
    description: 'UTC 16:00 应该转换为 UTC+8 次日 00:00'
  },
  {
    input: '2024-01-19T08:30:00',  // ISO 格式
    expected: '2024/01/19 16:30:00',
    description: 'ISO 格式应该正确转换'
  }
];

console.log('=== 时间格式化工具测试 ===\n');

testCases.forEach((testCase, index) => {
  const result = formatTimestamp(testCase.input);
  const passed = result === testCase.expected;
  
  console.log(`测试 ${index + 1}: ${testCase.description}`);
  console.log(`  输入: ${testCase.input}`);
  console.log(`  预期: ${testCase.expected}`);
  console.log(`  实际: ${result}`);
  console.log(`  结果: ${passed ? '✓ 通过' : '✗ 失败'}\n`);
});

console.log('=== 其他格式化函数测试 ===\n');
console.log('formatDate:', formatDate('2024-01-19 08:30:00'));
console.log('formatTime:', formatTime('2024-01-19 08:30:00'));
