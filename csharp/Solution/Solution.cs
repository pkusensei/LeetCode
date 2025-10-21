using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MaxFrequency(int[] nums, int k, int numOperations)
    {
        int max = nums.Max();
        int n = max + k + 2;
        int[] freq = new int[n];
        foreach (var item in nums)
        {
            freq[item] += 1;
        }
        int[] prefix = new int[n];
        prefix[0] = freq[0];
        for (int i = 1; i < n; i++)
        {
            prefix[i] = prefix[i - 1] + freq[i];
        }
        int res = 0;
        for (int num = 0; num < n; num++)
        {
            if (freq[num] == 0 && numOperations == 0) { continue; }
            int left = int.Max(0, num - k);
            int right = int.Min(n - 1, num + k);
            int f = prefix[right] - (left > 0 ? prefix[left - 1] : 0);
            int change = f - freq[num];
            int curr = freq[num] + int.Min(change, numOperations);
            res = int.Max(res, curr);
        }
        return res;
    }
}
