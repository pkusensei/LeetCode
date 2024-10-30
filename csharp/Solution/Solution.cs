using Solution.LList;
using Solution.Tree;

namespace Solution;

public class Solution
{
    public bool LemonadeChange(int[] bills)
    {
        var (five, ten) = (0, 0);
        foreach (var n in bills)
        {
            switch (n)
            {
                case 5: five += 1; break;
                case 10:
                    if (five > 0)
                    {
                        five -= 1; ten += 1;
                        break;
                    }
                    else { return false; }
                case 20:
                    if (five > 0 && ten > 0)
                    {
                        five -= 1; ten -= 1;
                        break;
                    }
                    else if (five >= 3) { five -= 3; break; }
                    else { return false; }
                default:
                    break;
            }
        }
        return true;
    }
}
