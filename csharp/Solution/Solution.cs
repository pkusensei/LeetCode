using System.Text;
using Solution.LList;
using Solution.Tree;
using static Solution.Utils;

namespace Solution;

public class Solution
{
    public bool ValidUtf8(int[] data)
    {
        for (int i = 0; i < data.Length;)
        {
            int d;
            if (data[i] <= 0b01_111_111) { d = 1; }
            else if ((data[i] >> 5) == 0b110) { d = 2; }
            else if ((data[i] >> 4) == 0b1_110) { d = 3; }
            else if ((data[i] >> 3) == 0b111_10) { d = 4; }
            else { return false; }
            if (i + d > data.Length || data[(i + 1)..(i + d)].Any(v => (v >> 6) != 0b10))
            {
                return false;
            }
            i += d;
        }
        return true;
    }
}
