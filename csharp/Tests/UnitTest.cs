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
        int[][] e = [[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]];
        bool[] online = [true, true, true, true];
        Assert.AreEqual(3, sol.FindMaxPathScore(e, online, 10));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] e = [[0, 1, 8]];
        bool[] online = [true, true];
        Assert.AreEqual(8, sol.FindMaxPathScore(e, online, 11));
    }

    [TestMethod]
    public void TestMethod3()
    {
        int[][] e = [[0, 1, 0], [0, 2, 7], [1, 3, 9], [0, 4, 7], [2, 4, 9], [3, 4, 2], [0, 3, 5], [2, 3, 3], [1, 4, 6], [1, 2, 0]];
        bool[] online = [true, true, true, true, true];
        Assert.AreEqual(0, sol.FindMaxPathScore(e, online, 5));
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}