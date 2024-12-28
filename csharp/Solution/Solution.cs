using System.Text;
using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public int[] MaxSumOfThreeSubarrays(int[] nums, int k)
    {
        var sum = nums.Take(k).Sum();
        // Prep all sums
        List<int> sums = [sum];
        for (int i = 1; i + k <= nums.Length; i++)
        {
            sum -= nums[i - 1];
            sum += nums[i + k - 1];
            sums.Add(sum);
        }
        // Max sum from left
        sum = sums.First();
        var maxi = 0;
        List<int> prefix = [maxi];
        foreach (var (i, item) in sums.Select((n, i) => (i, n)).Skip(1))
        {
            if (item > sum)
            {
                sum = item;
                maxi = i;
            }
            prefix.Add(maxi);
        }
        // Max sum from right
        sum = sums.Last();
        maxi = sums.Count - 1;
        List<int> suffix = [maxi];
        foreach (var (i, item) in Enumerable.Reverse(sums.Select((n, i) => (i, n))).Skip(1))
        {
            if (item >= sum) // to save smallest index with certain sum
            {
                sum = item;
                maxi = i;
            }
            suffix.Add(maxi);
        }
        suffix.Reverse();
        // Find middle i
        int[] res = [0, 0, 0];
        sum = 0;
        for (int i = k; i < sums.Count - k; i++)
        {
            var curr = sums[prefix[i - k]] + sums[i] + sums[suffix[i + k]];
            if (curr > sum)
            {
                sum = curr;
                res = [prefix[i - k], i, suffix[i + k]];
            }
        }
        return res;
    }
}