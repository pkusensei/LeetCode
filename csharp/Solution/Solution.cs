using System.Collections.Frozen;
using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int CountBinarySubstrings(string s)
    {
        int n = s.Length;
        List<int> arr = [];
        for (int left = 0; left < n;)
        {
            int right = left;
            for (; right < n && s[right] == s[left]; right += 1) { }
            arr.Add(right - left);
            left = right;
        }
        int res = 0;
        for (int i = 0; i < arr.Count - 1; i++)
        {
            res += int.Min(arr[i], arr[1 + i]);
        }
        return res;
    }
}
