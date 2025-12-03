using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(22, new[] { 73, 55, 36, 5, 55, 14, 9, 7, 72, 52 }, 32, 69)]
    public void TestMethod1(int exp, int[] n, int a, int b)
    {
        Assert.AreEqual(exp, sol.NumSubarrayBoundedMax(n, a, b));
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