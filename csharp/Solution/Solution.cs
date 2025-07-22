using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ContainsNearbyAlmostDuplicate(int[] nums, int indexDiff, int valueDiff)
    {
        Dictionary<int, int> window = [];
        for (int i = 0; i < nums.Length; i++)
        {
            int buc = BucNum(nums[i], valueDiff);
            if (!window.TryAdd(buc, nums[i])) { return true; }
            if (window.TryGetValue(buc - 1, out var v) && nums[i] - v <= valueDiff) { return true; }
            if (window.TryGetValue(buc + 1, out v) && v - nums[i] <= valueDiff) { return true; }
            if (window.Count > indexDiff)
            {
                int num = nums[i - indexDiff];
                buc = BucNum(num, valueDiff);
                window.Remove(buc);
            }
        }
        return false;

        static int BucNum(int num, int diff)
        {
            int b = num / (1 + diff);
            if (num < 0) { b -= 1; }
            return b;
        }
    }
}