using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public string LargestGoodInteger(string num)
    {
        ReadOnlySpan<char> res = [];
        for (int i = 0; i < num.Length - 2; i++)
        {
            if (num[i] == num[1 + i] && num[1 + i] == num[2 + i])
            {
                if (res.IsEmpty || res[0] < num[i]) { res = num.AsSpan()[i..(i + 3)]; }
                i += 2;
            }
        }
        return new(res);
    }
}