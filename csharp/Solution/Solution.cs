using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int MinMaxDifference(int num)
    {
        var s = num.ToString();
        int max = 0;
        char target = '9';
        foreach (var item in s)
        {
            if (target == '9' && item != '9') { target = item; }
            max *= 10;
            max += item == target ? 9 : item - '0';
        }
        int min = 0;
        target = '0';
        foreach (var item in s)
        {
            if (target == '0' && item != '0') { target = item; }
            min *= 10;
            min += item == target ? 0 : item - '0';
        }
        return max - min;
    }
}
