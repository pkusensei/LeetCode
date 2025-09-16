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
        int[][] n = [[4, 10, 15, 24, 26], [0, 9, 12, 20], [5, 18, 22, 30]];
        Assert.IsTrue(new[] { 20, 24 }.SequenceEqual(sol.SmallestRange(n)));
    }

    [TestMethod]
    public void TestMethod2()
    {
        int[][] n = [[1, 2, 3], [1, 2, 3], [1, 2, 3]];
        Assert.IsTrue(new[] { 1, 1 }.SequenceEqual(sol.SmallestRange(n)));
    }

    [TestMethod]
    public void TestMethod3()
    {
    }

    [TestMethod]
    public void TestMethod4()
    {
    }
}