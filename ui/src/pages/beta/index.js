import Image from "next/image";
import { TrashSimple, CaretDown, Plus, PencilSimple } from "phosphor-react";
import React, { Fragment, useEffect, useRef, useState } from "react";
import axios from "axios";
import { SERVER_ADDRESS } from "../../types/constants";
import { Dialog, Transition } from "@headlessui/react";
import ReactPaginate from "react-paginate";

export default function ShowcaseHN() {
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");
  const [success, setSuccess] = useState(false);
  const [submitted, setSubmitted] = useState(false);
  const [jobResults, setJobResults] = useState([]);
  const [jobCount, setJobCount] = useState(0);
  const [query, setQuery] = useState("");
  function Items({ currentItems }) {
    return (
      <>
        {currentItems &&
          currentItems.map((job) => (
            <li key={job.url} className="flex flex-wrap gap-2">
              <div className="flex flex-wrap gap-2 p-2 rounded place-items-center bg-stone-100">
                <div className="flex flex-col flex-wrap w-full gap-2 ">
                  <div className="flex-1 font-bold">{job.title}</div>
                  <div className="font-medium shrink-0">{job.summary}</div>
                  <a
                    href={`https://dantri.com.vn${job.url}`}
                    target="_blank"
                    className="flex-1 font-medium text-blue-600"
                  >
                    Xem thêm
                  </a>
                </div>
              </div>
            </li>
          ))}
      </>
    );
  }
  function PaginatedItems({ itemsPerPage }) {
    // Here we use item offsets; we could also use page offsets
    // following the API or data you're working with.
    const [itemOffset, setItemOffset] = useState(0);

    // Simulate fetching items from another resources.
    // (This could be items from props; or items loaded in a local state
    // from an API endpoint with useEffect and useState)
    const endOffset = itemOffset + itemsPerPage;
    console.log(`Loading items from ${itemOffset} to ${endOffset}`);
    const currentItems = jobResults.slice(itemOffset, endOffset);
    const pageCount = Math.ceil(jobResults.length / itemsPerPage);

    // Invoke when user click to request another page.
    const handlePageClick = (event) => {
      const newOffset = (event.selected * itemsPerPage) % jobResults.length;
      console.log(
        `User requested page number ${event.selected}, which is offset ${newOffset}`
      );
      setItemOffset(newOffset);
    };

    return (
      <>
        <ReactPaginate
          breakLabel="..."
          nextLabel="next >"
          onPageChange={handlePageClick}
          pageRangeDisplayed={3}
          pageCount={pageCount}
          previousLabel="< previous"
          renderOnZeroPageCount={null}
          containerClassName="flex justify-center my-4 w-full gap-2 text-sm font-medium text-slate-800 "
          pageClassName="mx-1 px-2 py-1 border rounded hover:bg-slate-800 hover:text-white"
          activeClassName="bg-slate-800 text-white"
          pageLinkClassName="hover:bg-slate-800 hover:text-white"
          disabledClassName="opacity-50 cursor-not-allowed"
          previousClassName="mx-1 px-2 py-1 border rounded"
          nextClassName="mx-1 px-2 py-1 border rounded"
          breakClassName="mx-1 px-2 py-1 border rounded"
        />
        <Items currentItems={currentItems} />
      </>
    );
  }

  const handleSubmit = async (e) => {
    e.preventDefault();
    setSubmitted(true);
    await search();
  };

  const search = async () => {
    setLoading(true);
    setSuccess(false);
    try {
      const res = await axios.post(`${SERVER_ADDRESS}/api/articles/query`, {
        query: query,
      });
      const data = res.data?.data;
      const result = { data: [] };
      data.forEach((item) => {
        result.data.push(JSON.parse(item));
      });
      setJobResults(result.data);
      setJobCount(res.data.article_count);
    } catch (error) {
      console.error(error);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="grid w-full max-w-4xl gap-4 p-2 mx-auto antialiased font-normal leading-relaxed bg-white auto-rows-min scroll-smooth text-slate-800 sm:p-4">
      <div className="flex w-full gap-2 bg-white border-2 rounded flex-nowrap place-items-center border-aubergine">
        <div className="flex-1 px-3 py-2 text-2xl font-medium text-aubergine">
          NewExpress
        </div>
      </div>
      <form className="flex flex-wrap gap-2">
        <input
          type="text"
          name="query"
          id="query"
          placeholder="Tìm kiếm"
          className="flex-1 p-2 border rounded"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
      </form>
      <hr />
      <button
        disabled={loading}
        onClick={handleSubmit}
        type="submit"
        className="btn-primary !p-3 text-xl !font-medium"
      >
        Tìm kiếm
      </button>
      <hr />
      {!loading && (
        <h3 className="text-xl font-medium">{jobCount} results found</h3>
      )}
      {/*<!-- block: ready to try? -->*/}
      <div className="">
        <div className="max-w-4xl">
          {loading && (
            <p className="text-center">
              Searching...
              <br /> Note: this can a few minutes...
            </p>
          )}
          {jobResults?.length > 0 && !loading && (
            <ul role="list" className="space-y-2">
              <PaginatedItems itemsPerPage={4} />,
            </ul>
          )}
          {jobResults?.length === 0 && loading === false && submitted && (
            <p className="text-center">No results</p>
          )}
        </div>
      </div>
    </div>
  );
}
