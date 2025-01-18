using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new[] { 1, 2, 3, 4, 5, 10, 6, 7, 8, 9 }, 5, true)]
    [DataRow(new[] { 1, 2, 3, 4, 5, 6 }, 7, true)]
    [DataRow(new[] { 1, 2, 3, 4, 5, 6 }, 10, false)]
    [DataRow(new[] { 2, 2, 2, 4 }, 4, false)]
    public void TestMethod1(int[] arr, int k, bool exp)
    {
        Assert.AreEqual(exp, sol.CanArrange(arr, k));
    }

    [TestMethod]
    public void TestMethod2()
    {
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