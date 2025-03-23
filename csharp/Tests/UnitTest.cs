using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    public void TestMethod1()
    {
        int[][] v = [[1, 2], [1, 4], [1, 5], [2, 6], [2, 3], [4, 6]];
        Assert.AreEqual(4, sol.MagnificentSets(6, v));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] v = [[1, 2], [2, 3], [3, 1]];
        Assert.AreEqual(-1, sol.MagnificentSets(3, v));
    }

    [TestMethod]
    public void TestMethod3()
    {
        int[][] v = [[67, 29], [13, 29], [77, 29], [36, 29], [82, 29], [54, 29], [57, 29], [53, 29], [68, 29], [26, 29], [21, 29], [46, 29], [41, 29], [45, 29], [56, 29], [88, 29], [2, 29], [7, 29], [5, 29], [16, 29], [37, 29], [50, 29], [79, 29], [91, 29], [48, 29], [87, 29], [25, 29], [80, 29], [71, 29], [9, 29], [78, 29], [33, 29], [4, 29], [44, 29], [72, 29], [65, 29], [61, 29]];
        Assert.AreEqual(57, sol.MagnificentSets(92, v));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}