using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int LenLongestFibSubseq(int[] arr)
    {
        int n = arr.Length;
        Dictionary<(int, int), int> dict = [];
        HashSet<int> seen = [];
        int res = 0;
        for (int right = 0; right < n; right++)
        {
            seen.Add(arr[right]);
            for (int left = right - 1; left >= 0; left -= 1)
            {
                int diff = arr[right] - arr[left];
                if (diff >= arr[left]) { break; }
                if (seen.Contains(diff))
                {
                    int curr = 1 + dict.GetValueOrDefault((diff, arr[left]), 2);
                    dict.Add((arr[left], arr[right]), curr);
                    res = int.Max(res, curr);
                }
            }
        }
        return res;
    }
}