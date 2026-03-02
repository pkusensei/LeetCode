using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int TotalFruit(int[] fruits)
    {
        Dictionary<int, int> freq = [];
        int left = 0;
        int res = 0;
        for (int right = 0; right < fruits.Length; right++)
        {
            if (!freq.TryAdd(fruits[right], 1)) { freq[fruits[right]] += 1; }
            while (freq.Count > 2)
            {
                freq[fruits[left]] -= 1;
                if (freq[fruits[left]] == 0) { freq.Remove(fruits[left]); }
                left += 1;
            }
            res = int.Max(res, right + 1 - left);
        }
        return res;
    }
}