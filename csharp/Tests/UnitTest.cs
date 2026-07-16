using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(32, new[] { 6, 2, 4 })]
    public void TestMethod1(int exp, int[] arr)
    {
        Assert.AreEqual(exp, sol.MctFromLeafValues(arr));
        Assert.AreEqual(exp, sol.WithMonoStack(arr));
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