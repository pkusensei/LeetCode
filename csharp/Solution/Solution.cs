using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public int Maximum69Number(int num)
    {
        var s = num.ToString().ToCharArray();
        for (int i = 0; i < s.Length; i++)
        {
            if (s[i] == '6')
            {
                s[i] = '9';
                break;
            }
        }
        string s_ = new(s);
        return int.Parse(s_);
    }
}