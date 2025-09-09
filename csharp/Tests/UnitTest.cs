using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(5, new[] { 2, 6, 4, 8, 10, 9, 15 })]
    public void TestMethod1(int exp, int[] n)
    {
        Assert.AreEqual(exp, sol.FindUnsortedSubarray(n));
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