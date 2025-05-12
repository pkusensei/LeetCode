using Solution;
using Solution.LList;
using Solution.Tree;

namespace Tests;

[TestClass]
public class UnitTest
{
    readonly Solution.Solution sol = new();

    [TestMethod]
    [DataRow(new int[] { 2, 1, 3, 0 }, "[102,120,130,132,210,230,302,310,312,320]")]
    public void TestMethod1(int[] digits, string exp)
    {
        Assert.AreEqual(exp, sol.FindEvenNumbers(digits).Print());
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